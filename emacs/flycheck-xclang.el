;;; flycheck-xclang.el --- An Objective-C syntax checked using xclang

;; Author: Arthur Evstifeev

;;; Commentary:
;;

;;; Code:

(flycheck-define-checker xclang
  "An Objective-C syntax checker using xclang-tool."
  :command ("xclang" "syntax-check" "-o" source-original source)
  :error-patterns
  ((error line-start (file-name) ":" line ":" column ": "
          "error: " (message) line-end)
   (warning line-start (file-name) ":" line ":" column ": "
            "warning: " (message) line-end))
  :modes objc-mode)

(add-to-list 'flycheck-checkers 'xclang)

;;; flycheck-xclang.el ends here
