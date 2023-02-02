## Agenda
The server must have:
* Templates.
* Redirection on Errors.
* Sessions.
* Error Logging System.
* Cookies

## TLS
To create tls certificate and key files install [`mkcert.`]: https://github.com/FiloSottile/mkcert
Run `mkcert localhost 127.0.0.1` it will create sertificatate and key suitable for testing
and developmen.  
Current certificates are valid until 2025-04-02.  
When site is accessed browser will throw warning about certificate. Because certificate is not
issued by known autohority.

## Sqlite
For sqlite3 access must install libsqlite3-dev package.  
`sudo apt install libsqlite3-dev`
