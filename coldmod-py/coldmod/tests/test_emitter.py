from time import sleep
import coldmod.tracing
from introspection.udp_listener import UDPListener
from coldmod.samples.app_1 import app
from coldmod_msg_py import Trace
from pprint import pp

def _trace_to_string(bytes: bytes):
    trace = Trace.Trace.GetRootAs(bytes)
    path = trace.Path()
    assert path is not None
    path = path.decode()

    return f"{path}:{trace.Line()}"

def test_emitter():
    host, port = ["127.0.0.1", 7777]

    udp_listener = UDPListener(host=host, port=port)
    udp_listener.start()

    try:
        coldmod.tracing.install(host=host, port=port)
        app.run()
        coldmod.tracing.uninstall()
        sleep(0.01) # yield to listener thread
    finally:
        udp_listener.stop()

    msg_bins: list = udp_listener.get_messages()

    msgs = list(map(_trace_to_string, msg_bins))

    assert len(msgs) == 6

    a,b,c,d,e,_ = msgs

    assert a.endswith("coldmod/samples/app_1/app.py:11")
    assert b.endswith("coldmod/samples/app_1/app.py:7")
    assert c.endswith("coldmod/samples/app_1/helper.py:6")
    assert d.endswith("coldmod/samples/app_1/helper.py:10")
    assert e.endswith("coldmod/samples/app_1/helper.py:2")
