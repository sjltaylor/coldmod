import os
import coldmod_py
import coldmod_py.source as source
import coldmod_py.files as files
import coldmod_py.config
import coldmod_py.tracing.src as src
import fire # https://github.com/google/python-fire/blob/master/docs/guide.md
import logging
import sys
from typing import List

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

    def tracing_src(self):
        """
        print the src scan used to generate the coldmod tracing
        """
        paths = files.find_src_files_in(self.config.srcs_root_dir, self.config.ignore_patterns)
        for tracing_src in src.find_tracing_srcs_in(paths):
            print(f"{tracing_src.name}:{tracing_src.digest}\n{tracing_src.path}:{tracing_src.lineno}\n")

if __name__ == "__main__":
    fire.Fire(CLI)
