class Helper():
    def message(self):
        return "hello"

class NameHelper(Helper):
    def __init__(self, name) -> None:
        self.name = name
        super().__init__()

    def message(self):
        return f"{super().message()} {self.name}"

def this_isnt_called_is_it(): #noqa
    pass