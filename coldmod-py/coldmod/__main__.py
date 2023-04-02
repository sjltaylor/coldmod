import os
from coldmod.read_sources import read_sources, list_sources, FnSource
from coldmod.read_trace import read_trace
from coldmod.heatmap import create_heatmap

COLDMOD = "COLD" + ''.join([c + '\u0336' for c in 'MOD'])

def main():
    [tracefile, source_path] = os.sys.argv[1:3]

    call_traces = read_trace(tracefile)
    fn_sources = read_sources(list_sources(source_path))

    heatmap = create_heatmap(call_traces, fn_sources)

    print(COLDMOD)
    for f in heatmap.not_called:
        print(f'{f.file}:{f.lineno}')

if __name__ == "__main__":
    main()