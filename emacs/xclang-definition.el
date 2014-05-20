;;; xclang-definition.el --- xclang based plugin to jump to the definition from the cursor position

;; Author: Arthur Evstifeev

;;; Commentary:
;;

;;; Code:

(defcustom xclang-definition-executable
  (executable-find "/Users/arthurevstifeev/github/xclang_tool/bin/xclang")
  "Location of xclang executable.")

(defun xclang--parse-output ()
  (let (line_split file line column)
    (setq line_split (split-string (buffer-string) ":"))
    (setq file (car line_split))
    (setq line (string-to-number (cadr line_split)))
    (setq column (string-to-number (car (last line_split))))
    (if (string= "" file)
        (message "Unable to find definition")
      (find-file file)
      (goto-line line)
      (move-to-column (- column 1)))
    ))

(defun xclang-definition ()
  (interactive)
  (and (buffer-modified-p)
       (basic-save-buffer))
  (let ((process-res 0)
        location file))
  (setq location (format "%d:%d"
                         (line-number-at-pos)
                         (current-column)))
  (setq file buffer-file-name)
  (with-temp-buffer
    (setq process-res (call-process xclang-definition-executable nil t nil "goto-definition" "-l" location file))
    (xclang--parse-output)))

(provide 'xclang-definition)
;;; xclang-definition.el ends here
