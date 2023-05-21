import fire # https://github.com/google/python-fire/blob/master/docs/guide.md

class CLI:
    class Source:
        def send(self):
            print("Sending...")

    def source(self):
        return self.Source()

if __name__ == "__main__":
    fire.Fire(CLI)
