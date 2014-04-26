;;; company-xclang.el --- company-mode completion back-end for xclang

;; Author: Arthur Evstifeev

;;; Commentary:
;;

;;; Code:

(require 'cl-lib)
(require 'yasnippet)

(defcustom company-xclang-executable
  (executable-find "/Users/arthurevstifeev/github/xclang_tool/bin/xclang")
  "Location of xclang executable."
  :type 'file)

(defvar company-xclang-modes '(objc-mode)
  "Major modes in which xclang may complete.")

(defun company-xclang--make-candidate (candidate)
  (let ((text (car candidate))
        (meta (cadr candidate)))
    (propertize text 'meta meta)))

(defun company-xclang--parse-output ()
  (let (lines)
    (dolist (line (split-string (buffer-string) "\n" t))
      (push (company-xclang--make-candidate (split-string line "\t"))
            lines))
    lines))

(defun company-xclang--candidates (prefix)
  (and (buffer-modified-p)
       (basic-save-buffer))
  (let ((process-res 0)
        results location file search))
  (setq location (format "%d:%d"
                         (line-number-at-pos)
                         (current-column)))
  (setq file buffer-file-name)
  (setq search (substring-no-properties prefix))
  (with-temp-buffer
    (setq process-res (call-process company-xclang-executable nil t nil "-p" search "-c" location file))
    (company-xclang--parse-output)))

(defun company-xclang--annotation (candidate)
  (format " (%s)" (get-text-property 0 'meta candidate)))

(defun company-xclang--expand (candidate)
  (let ((start (- (point) (length candidate)))
        (stop (point-marker)))
    (delete-region start stop))
  (yas-expand-snippet candidate))

(defun company-xclang (command &optional arg &rest ignored)
  "`company-mode' completion back-end for xclang."
  (interactive (list 'interactive))
  (cl-case command
    (interactive (company-begin-backend 'company-xclang))
    (prefix (and (memq major-mode company-xclang-modes)
                 (not (company-in-string-or-comment))
                 (company-grab-symbol)))
    (candidates (company-xclang--candidates arg))
    (annotation (company-xclang--annotation arg))
    (post-completion (company-xclang--expand arg))
    ))

(push 'company-xclang company-backends)

(provide 'company-xclang)
;;; company-xclang.el ends here
