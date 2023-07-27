import queue
from typing import Iterable, Tuple
import threading

Q = queue.Queue(maxsize=65536)
# could use a deque here instead, but reading from this doesn't block, we'd have to

def generator() -> Iterable[Tuple[str,int, int, int]]:
    while True:
        yield Q.get()
