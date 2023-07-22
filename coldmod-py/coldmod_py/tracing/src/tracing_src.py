class TracingSrc():
    path: str
    name: str
    lineno: int
    digest: str

    def __init__(self, path: str, name: str, lineno: int, digest: str):
        self.path = path
        self.name = name
        self.lineno = lineno
        self.digest = digest
