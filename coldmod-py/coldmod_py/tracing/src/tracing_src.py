class TracingSrc():
    path: str
    name: str
    class_name_path: str
    lineno: int
    digest: str
    src: str

    def __init__(self, *, path: str, lineno: int, name: str, class_name_path, src: str, digest: str):
        self.path = path
        self.lineno = lineno
        self.name = name
        self.class_name_path = class_name_path
        self.src = src
        self.digest = digest
