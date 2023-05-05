import socket
import threading
import time
from collections import deque
from typing import Iterable

class UDPListener:
    def __init__(self, *, host, port):
        self.host = host
        self.port = port
        self.stop_event = threading.Event()
        self.buffer = deque()
        self.listener_thread = threading.Thread(target=self._listener)

    def start(self):
        self.listener_thread.start()

    def stop(self):
        self.stop_event.set()
        self.listener_thread.join()

    def _listener(self):
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        sock.bind((self.host, self.port))
        sock.settimeout(0.1)

        while True:
            try:
                data, _ = sock.recvfrom(1024)
                self.buffer.append(data)
            except socket.timeout:
                pass
            if self.stop_event.is_set():
                break

        sock.close()

    def get_messages(self) -> list:
        return list(self.buffer)
