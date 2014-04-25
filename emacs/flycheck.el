(flycheck-define-checker xclang
  "An Objective-C syntax checker using xclang-tool."
  :command ("xclang"
            "-o" source-original
            "-s" source)
  :error-patterns
  ((error line-start (file-name) ":" line ":" column ": "
          "error: " (message) line-end)
   (warning line-start (file-name) ":" line ":" column ": "
            "warning: " (message) line-end))
  :modes objc-mode)

(add-to-list 'flycheck-checkers 'xclang)
