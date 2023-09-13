import os
import coldmod_py
import coldmod_py.files as files
import coldmod_py.config
import coldmod_py.code as code
import coldmod_py.cache as cache
import coldmod_py.mod as mod
import fire # https://github.com/google/python-fire/blob/master/docs/guide.md
import logging
import sys
import webbrowser
from typing import List
from google.protobuf.json_format import MessageToDict, ParseDict
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import json
import queue

class CLI:
    class Cache():
        def clear(self):
            """
            clear the coldmod cache
            """
            cache.clear()

    def __init__(self, path=None, verbose=False):
        if verbose:
            logging.basicConfig(level=logging.DEBUG)

    def cache(self):
        """
        coldmod cache commands
        """
        return self.Cache()

    def trace_srcs(self):
        """
        print traces
        """
        root_marker = coldmod_py.root_marker.load()
        paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())

        relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]
        trace_srcs_by_relative_path = code.find_trace_srcs(relative_paths)

        for relative_path, trace_srcs in trace_srcs_by_relative_path.items():
            print(relative_path)
            for p in trace_srcs:
                print(f"  {p.trace_src.key}")

    def src_files(self):
        """
        print the files which are included in coldmod tracing
        """
        root_marker = coldmod_py.root_marker.load()
        for path in files.find_src_files_in(root_marker.dir(), root_marker.ignore_files()):
            print(path)

    def connect(self, web_app_url=None):
        root_marker = coldmod_py.root_marker.load()

        if web_app_url is None:
            (web_app_url, key) = coldmod_py.web.generate_app_url()
            print(f"connect to: {web_app_url}")
            webbrowser.open(web_app_url)
        else:
            key = coldmod_py.web.extract_key(web_app_url)

        paths = files.find_src_files_in(os.getcwd(), root_marker.ignore_files())

        relative_paths = [os.path.relpath(p, os.getcwd()) for p in paths]
        trace_srcs_by_relative_path = code.find_trace_srcs(relative_paths)

        trace_srcs_by_key = { p.trace_src.key:p for (_,srcs) in trace_srcs_by_relative_path.items() for p in srcs}

        create_src_ignore_key_message = lambda key: tracing_pb2.SrcMessage(src_ignore=tracing_pb2.SrcIgnore(key=key))
        create_src_available_message = lambda key: tracing_pb2.SrcMessage(src_available=tracing_pb2.SrcAvailable(key=key))

        src_message_queue: queue.Queue[tracing_pb2.SrcMessage] = queue.Queue(maxsize=256)

        connect = tracing_pb2.SrcMessage(connect_key=tracing_pb2.ConnectKey(key=key))

        src_message_queue.put(connect)

        for cmd in coldmod_py.web.stream_commands(src_message_queue):
            match cmd.WhichOneof("command"):
                case "send_src_info":
                    for key in root_marker.ignore_keys():
                        src_message_queue.put(create_src_ignore_key_message(key))
                    for key in trace_srcs_by_key.keys():
                        src_message_queue.put(create_src_available_message(key))
                case "ignore":
                    root_marker.add_ignore_key(cmd.ignore.key).dump()
                    ignore = create_src_ignore_key_message(cmd.ignore.key)
                    src_message_queue.put_nowait(ignore)
                case _:
                    print(f"command not supported: {cmd}")


    def mod_remove(self, force=False):
        root_marker = coldmod_py.root_marker.load()

        if not force:
            print("Are you sure (y/N)? (you didn't use --force)")
            yN = input()
            if yN != "y":
                print("aborting")
                sys.exit(1)

        with open('./coldmod.filterset.json', 'r') as json_file:
            raw = json_file.read()
            trace_srcs = ParseDict(json.loads(raw), tracing_pb2.TraceSrcs())
            src_files = files.find_src_files_in(root_marker.dir(), root_marker.ignore_files())
            mod.remove(root_marker.dir(), trace_srcs.trace_srcs, src_files)


if __name__ == "__main__":
    try:
        fire.Fire(CLI)
    except KeyboardInterrupt:
        pass
