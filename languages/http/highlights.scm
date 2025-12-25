; Highlight HTTP methods
(method) @function.method

; Highlight HTTP comments
(comment) @comment

; Highlight URLs and paths
(target_url) @string.url
(host) @string.url
(path) @string.url

; Highlight HTTP headers
(header name: (name) @property)
(header value: (value) @string)

; Highlight HTTP status codes and status texts
(status_code) @constant.numeric
(status_text) @constant.language

; Highlight HTTP versions
(http_version) @keyword

; Highlight variable declarations (@foo=bar)
(variable_declaration
  name: (identifier) @variable)
(variable_declaration
  value: (string) @string)
(variable_declaration
  value: (number) @number)
(variable_declaration
  value: (boolean) @boolean)

; Highlight variables and script variables ({{foo}})
(variable) @variable
(script_variable) @variable.special

; Highlight different types of request bodies
(json_body) @string.special
(xml_body) @string.special
(graphql_body) @string.special
(external_body) @string.special
(form_data) @string.special

; Highlight query parameters
(query_param) @string
