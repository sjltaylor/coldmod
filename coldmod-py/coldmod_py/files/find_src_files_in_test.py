import os
import coldmod_py

def test_find_srcs():
    module_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    example_app_path = os.path.join(module_dir, "samples", "trace_target_1")

    file_list = coldmod_py.files.find_src_files_in(example_app_path, [
        "files/**/*",
        "not_me.py"])

    file_1 = os.path.join(example_app_path, "__main__.py")
    file_2 = os.path.join(example_app_path, "helper.py")
    ignore_1 = os.path.join(example_app_path, "not_me.py")
    ignore_2 = os.path.join(example_app_path, "files/down/here/are", "not_to_be_included.py")

    assert file_1 in file_list
    assert file_2 in file_list
    assert ignore_1 not in file_list
    assert ignore_2 not in file_list

def test_find_srcs_without_ignores():
    module_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    example_app_path = os.path.join(module_dir, "samples", "trace_target_1")

    file_list = coldmod_py.files.find_src_files_in(example_app_path)

    file_1 = os.path.join(example_app_path, "__main__.py")
    file_2 = os.path.join(example_app_path, "helper.py")
    file_3 = os.path.join(example_app_path, "not_me.py")
    file_4 = os.path.join(example_app_path, "files/down/here/are", "not_to_be_included.py")

    assert file_1 in file_list
    assert file_2 in file_list
    assert file_3 in file_list
    assert file_4 in file_list
