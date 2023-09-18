import os
import coldmod_py
from coldmod_py.code import parsed_trace_src
import coldmod_py.files as files
import coldmod_py.config
import coldmod_py.code as code
import coldmod_py.cache as cache
import coldmod_py.mod as mod
import coldmod_py.coldmod_d
import fire # https://github.com/google/python-fire/blob/master/docs/guide.md
import logging
import sys
import webbrowser
from typing import List, Tuple
from google.protobuf.json_format import MessageToDict, ParseDict
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import json
import queue
import threading

class CLI:
    class Cache():
        def clear(self):
            """
            clear the coldmod cache
            """
            cache.clear()

        def warm(self):
            """
            warm the coldmod cache
            """
            root_marker = coldmod_py.root_marker.load()
            paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())

            relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]
            trace_srcs_by_relative_path = code.find_trace_srcs_by_relative_paths(relative_paths)
            print("done.")


    def __init__(self, path=None, verbose=False):
        if verbose:
            logging.basicConfig(level=logging.DEBUG)

    def cache(self):
        """
        coldmod cache commands
        """
        return self.Cache()

    def srcs(self):
        """
        print trace srcs
        """
        root_marker = coldmod_py.root_marker.load()
        paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())

        relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]
        trace_srcs_by_relative_path = code.find_trace_srcs_by_relative_paths(relative_paths)

        for relative_path, trace_srcs in trace_srcs_by_relative_path.items():
            print(relative_path)
            for p in trace_srcs:
                print(f"  {p.trace_src.key}")

    def files(self):
        """
        print the files which are parsed for trace srcs
        """
        root_marker = coldmod_py.root_marker.load()
        for path in files.find_src_files_in(root_marker.dir(), root_marker.ignore_files()):
            print(path)

    def connect(self, web_app_url=None):
        root_marker = coldmod_py.root_marker.load()

        if web_app_url is None:
            (web_app_url, connect_key) = coldmod_py.web.generate_app_url()
            print(f"connect to: {web_app_url}")
            webbrowser.open(web_app_url)
        else:
            connect_key = coldmod_py.web.extract_key(web_app_url)

        paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())

        relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]
        parsed_trace_srcs = code.by_key(code.find_trace_srcs_by_relative_paths(relative_paths))

        create_src_ignore_key_message = lambda key: tracing_pb2.SrcMessage(src_ignore=tracing_pb2.SrcIgnore(key=key))
        create_src_available_message = lambda keys: tracing_pb2.SrcMessage(src_available=tracing_pb2.SrcAvailable(keys=keys))
        create_src_remove_result_message = lambda key,success: tracing_pb2.SrcMessage(src_remove_result=tracing_pb2.SrcRemoveResult(key=key,success=success))

        src_message_queue: queue.Queue[tracing_pb2.SrcMessage] = queue.Queue(maxsize=256)

        connect = tracing_pb2.SrcMessage(connect_key=tracing_pb2.ConnectKey(key=connect_key))

        src_message_queue.put(connect)

        stop_event = None
        src_refs_thread = None

        for cmd in coldmod_py.web.stream_commands(src_message_queue):
            match cmd.WhichOneof("command"):
                case "send_src_info":
                    for key in root_marker.ignore_keys():
                        src_message_queue.put(create_src_ignore_key_message(key))
                    src_message_queue.put(create_src_available_message(list(parsed_trace_srcs.keys())))

                    if stop_event is not None:
                        stop_event.set()
                    if src_refs_thread is not None:
                        src_refs_thread.join()

                    stop_event = threading.Event()

                    src_refs_thread = threading.Thread(target=mod.queue_src_refs, args=[root_marker.dir(), parsed_trace_srcs.values(), src_message_queue, stop_event], daemon=True)
                    src_refs_thread.start()

                case "open":
                    editor = os.getenv("EDITOR")
                    if editor is None:
                        print("EDITOR environment variable not set")
                        continue
                    parsed_trace_src_and_path = parsed_trace_srcs.get(cmd.open.key)
                    if parsed_trace_src_and_path is None:
                        print(f"not found: {cmd.open.key}")
                        continue
                    (parsed_trace_src, path) = parsed_trace_src_and_path
                    os.system(f"{editor} {path}:{parsed_trace_src.position.line}")

                case "ignore":
                    root_marker.add_ignore_key(cmd.ignore.key).dump()
                    ignore = create_src_ignore_key_message(cmd.ignore.key)
                    src_message_queue.put_nowait(ignore)
                case "remove":
                    (parsed_trace_src, path) = parsed_trace_srcs[cmd.remove.key]
                    success = True
                    try:
                        mod.remove(root_marker.dir(), parsed_trace_src, path)
                    except Exception as e:
                        success = False
                        print(f"failed to remove {parsed_trace_src.trace_src.key}: {e}")
                    msg = create_src_remove_result_message(parsed_trace_src.trace_src.key, success)
                    src_message_queue.put_nowait(msg)

                    parsed_trace_srcs = code.by_key(code.find_trace_srcs_by_relative_paths(relative_paths))

                    # TODO: consider moving this to a function
                    if stop_event is not None:
                        stop_event.set()
                    if src_refs_thread is not None:
                        src_refs_thread.join()

                    stop_event = threading.Event()

                    src_refs_thread = threading.Thread(target=mod.queue_src_refs, args=[root_marker.dir(), parsed_trace_srcs.values(), src_message_queue, stop_event], daemon=True)
                    src_refs_thread.start()

                case _:
                    print(f"command not supported: {cmd}")


    def rm(self, *keys: str):
        """
        Remove functions corresponding to the given keys
        """
        root_marker = coldmod_py.root_marker.load()

        paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())

        relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]

        for key in keys:
            parsed_trace_srcs = code.by_key(code.find_trace_srcs_by_relative_paths(relative_paths))

            (parsed_trace_src, path) = parsed_trace_srcs[key]
            mod.remove(root_marker.dir(), parsed_trace_src, path)

    def ignore(self, *keys: str):
        """
        Add the given keys to the ignore list in coldmod.rootmarker
        """

        root_marker = coldmod_py.root_marker.load()

        for key in keys:
            root_marker.add_ignore_key(key)

        root_marker.dump()

    def fetch(self, all = False, format = "text"):
        """
            fetch a snapshot of tracing data and reconcile with local source code.
            anything that can't be found locally is not included in json output.
        """
        heat_map = coldmod_py.coldmod_d.fetch(all)
        root_marker = coldmod_py.root_marker.load()
        paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())
        relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]
        parsed_trace_srcs = code.by_key(code.find_trace_srcs_by_relative_paths(relative_paths))

        match format:
            case "json":
                matching_srcs = [(p[0], p[1], c) for (p, c) in [(parsed_trace_srcs.get(src.key), src.trace_count) for src in heat_map.srcs] if p is not None]
                dicts = [{
                    'key': src.trace_src.key,
                    'loc': f"{path.absolute()}:{src.position.line}",
                    'traces': count
                } for (src, path, count) in matching_srcs]
                print(json.dumps(dicts, indent=2))
            case "text":
                for src in heat_map.srcs:
                    print(f"key:{src.key}")
                    print(f"traces:{src.trace_count}")
                    parsed_trace_src_and_path = parsed_trace_srcs.get(src.key)
                    if parsed_trace_src_and_path is None:
                        print("lod:[not found]")
                    else:
                        (parsed_trace_src, path) = parsed_trace_src_and_path
                        print(f"loc:{path.absolute()}:{parsed_trace_src.position.line}")

                    print("\n")
            case _:
                print("format not supported")

if __name__ == "__main__":
    try:
        fire.Fire(CLI)
    except KeyboardInterrupt:
        pass
