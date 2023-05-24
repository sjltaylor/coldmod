import os
import coldmod_py.source as source
import fire # https://github.com/google/python-fire/blob/master/docs/guide.md

class CLI:
    def send(self, *, url=os.environ.get("COLDMOD_D_URL"), path=os.getcwd()):
        assert url is not None, "set COLDMOD_D_URL or pass it as an argument"
        print(f"coldmod daemon: {url}")
        print(f"source directory: {path}")
        source_paths = source.files.find_in(path)
        sources = source.files.read_all(source_paths)
        root_marker_paths = list(source.scan.find_root_markers(sources))
        source_prefix = path

        if len(root_marker_paths) == 0:
            print(f"no root markers found, using {path} as source prefix")
        elif len(root_marker_paths) == 1:
            source_prefix = root_marker_paths[0]
        else:
            print("error: multiple root markers found:\n")
            print(root_marker_paths)
            return

        modules = source.scan.parse_all(sources)
        source_fns = source.scan.find_functions_in_all(modules)

        source.connect.submit_source_scan(source_prefix, source_fns)

if __name__ == "__main__":
    fire.Fire(CLI)