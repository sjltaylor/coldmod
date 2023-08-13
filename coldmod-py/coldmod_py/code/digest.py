import ast
from hashlib import blake2b

def function_def_digest(src: str, rel_module_path: str, class_name_path: str|None = None):
    stripped_src = ast.unparse(ast.parse(src))
    buffer = [rel_module_path]
    buffer.append(f"[{class_name_path or ''}]")
    buffer.append(stripped_src)
    return blake2b("".join(buffer).encode('utf-8')).hexdigest()
