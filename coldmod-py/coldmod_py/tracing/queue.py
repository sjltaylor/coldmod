import queue
from typing import Iterable, Tuple
import threading
Q = queue.Queue(maxsize=65536)

def generator() -> Iterable[Tuple[str,int]]:
    while True:
        yield Q.get()
