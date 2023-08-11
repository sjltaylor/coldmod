import os
import coldmod_py
import coldmod_py.files as files
import coldmod_py.config
import coldmod_py.code as code
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
    def __init__(self, path=None, verbose=False):
        if verbose:
            logging.basicConfig(level=logging.DEBUG)
        self.config = coldmod_py.config.load(path)

    def list_srcs(self):
        """
        print the files which are included in coldmod tracing
        """
        for path in files.find_src_files_in(self.config.srcs_root_dir, self.config.ignore_patterns):
            print(path)

    def heat_srcs(self):
        """
        print the src scan used to generate the coldmod tracing
        """
        paths = files.find_src_files_in(self.config.srcs_root_dir, self.config.ignore_patterns)
        trace_srcs = code.find_trace_srcs_in(self.config.srcs_root_dir, paths)
        for trace_src in trace_srcs:
            print(f"{trace_src.name}:{trace_src.digest}\n{trace_src.path}:{trace_src.lineno}\n")

    def duplicates(self):
        """
        print any srcs that have the same digest
        """
        paths = files.find_src_files_in(self.config.srcs_root_dir, self.config.ignore_patterns)
        trace_srcs = code.find_trace_srcs_in(self.config.srcs_root_dir, paths)
        duplicates = code.duplicates(trace_srcs)
        for digest, duplicate_trace_srcs in duplicates.items():
            print(f"{len(list(duplicate_trace_srcs))} => {digest}\n")
            for trace_src in duplicate_trace_srcs:
                print(f"{trace_src.name} -> {trace_src.path}:{trace_src.lineno}\n")
                print(f"{trace_src.src}\n")

    def connect(self, web_app_url=None):
        if web_app_url is None:
            (web_app_url, key) = coldmod_py.web.generate_app_url()
            print(f"connect to: {web_app_url}")
            webbrowser.open(web_app_url)
        else:
            key = coldmod_py.web.extract_key(web_app_url)

        path_prefix = self.config.srcs_root_dir

        previous_lines = []

        for filterset in coldmod_py.web.stream_filterset(key):
            with open('./coldmod.filterset.json', 'w') as json_file:
                raw = json.dumps(MessageToDict(filterset), indent=4)
                json_file.write(raw)
            with open('./coldmod.filterset.locs.txt', 'w') as locs_file:
                for line in previous_lines:
                    sys.stdout.write("\033[F")
                    sys.stdout.write("\r")
                    sys.stdout.write(" " * len(line))
                    sys.stdout.write("\r")
                    sys.stdout.flush()

                previous_lines = []

                for trace_src in filterset.trace_srcs:
                    loc = f"{path_prefix}/{trace_src.path}:{trace_src.lineno}\n"
                    locs_file.write(loc)
                    sys.stdout.write(loc)
                    sys.stdout.flush()
                    previous_lines.append(loc)

                previous_lines.reverse()



    def mod_remove(self, force=False):
        if not force:
            print("Are you sure (y/N)? (you didn't use --force)")
            yN = input()
            if yN != "y":
                print("aborting")
                sys.exit(1)

        with open('./coldmod.filterset.json', 'r') as json_file:
            raw = json_file.read()
            filterset = ParseDict(json.loads(raw), tracing_pb2.FilterSet())
            mod.remove(filterset)



if __name__ == "__main__":
    try:
        fire.Fire(CLI)
    except KeyboardInterrupt:
        pass
