((json_body) @injection.content
  (#set! injection.language "json"))

((xml_body) @injection.content
  (#set! injection.language "xml"))

((graphql_body) @injection.content
  (#set! injection.language "graphql"))
