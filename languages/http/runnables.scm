; HTTP request runnables for Zed editor
; This enables running HTTP requests directly from .http files

(
  (request
    (method) @run
  ) @http-request
  (#set! tag http-request)
)
