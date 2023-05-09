import threading
import time
import random
import coldmod_py.tracing
from pprint import pprint as pp

def work():
    print('work')

def app_worker():
    for i in range(10):
        work()


def threads_inspector():
    for i in range(10):
        pp(threading.enumerate())
        time.sleep(1)

def main():
    threading.Thread(target=threads_inspector, daemon=True).start()
    time.sleep(0.1)
    coldmod_py.tracing.start_in_this_dir()

    work()
    work()
    work()
    work()
    work()

    for i in range(4):
        t = threading.Thread(target=app_worker)
        t.start()

    time.sleep(1)

if __name__ == '__main__':
    main();
