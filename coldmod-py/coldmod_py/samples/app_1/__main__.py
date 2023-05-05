import coldmod_py.tracing
import coldmod_py.samples.app_1.app as app

coldmod_py.tracing.install(host="127.0.0.1", port=7777)
app.run()
