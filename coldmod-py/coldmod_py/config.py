import os

class Env:
    def grpc_host(self) -> str:
        host = os.getenv("COLDMOD_GRPC_HOST")
        if not host:
            raise Exception("COLDMOD_GRPC_HOST not set")
        return host

    def ca(self) -> str:
        ca = os.getenv("COLDMOD_TLS_CA")
        if not ca:
            raise Exception("COLDMOD_TLS_CA not set")
        return ca

    def web_app_url(self) -> str:
        key = 'COLDMOD_WEB_HOST'
        protocol = 'https'

        if self.insecure():
            key = 'COLDMOD_APP_HOST'
            protocol = 'http'

        web_host = os.getenv(key)
        if not web_host:
            raise Exception(f"{key} not set")

        return f"{protocol}://{web_host}"

    def api_key(self) -> str:
        api_key = os.getenv("COLDMOD_API_KEY")
        if not api_key:
            raise Exception("COLDMOD_API_KEY not set")
        return api_key

    def insecure(self) -> bool:
        insecure = os.getenv("COLDMOD_INSECURE") == "on"
        return insecure


env = Env()
