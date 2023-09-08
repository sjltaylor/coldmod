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

        for cmd in coldmod_py.web.stream_commands(key):
            match cmd.WhichOneof("command"):
                case "ignore":
                    print("ignoring:", cmd.ignore.key)
                    root_marker.add_ignore_key(cmd.ignore.key).dump()
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
