import coldmod_py
import os

def test_read_all():
    paths = [os.path.abspath(os.path.join(__file__, "../../samples/trace_target_1", path)) for path in ['__main__.py', 'helper.py']]
    result = coldmod_py.files.read_all(paths)
    assert len(result) == 2
    with open(paths[0], "r") as f:
        assert result[paths[0]] == f.read()
