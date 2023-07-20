import os
import coldmod_py
import coldmod_py.source as source
import coldmod_py.fs as fs
import coldmod_py.config
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
        for path in fs.find_srcs_in(self.config.srcs_root_dir, self.config.ignore_patterns):
            print(path)

if __name__ == "__main__":
    fire.Fire(CLI)
