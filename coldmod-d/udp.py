import io
import socket

for i in range(1000):
    socket.socket(socket.AF_INET, socket.SOCK_DGRAM).sendto(f"HELLO:{i}".encode("utf8"), ("127.0.0.1", 7777))
