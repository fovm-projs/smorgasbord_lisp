# Smorgasbord Lisp

<p align="center">  
  <img src="./images/logo.svg" width="512">
</p>

Smorgasbord Lisp is a system programming language designed to be an IR

## Examples

Currently working examples

### Hello, World!

```
(write 0 "Hello, World!")
```

### Factorial

```
(defun factorial (n)
  (if (eq n 0)
    1
    (mul n (factorial (sub n 1)))))

(write 0 (factorial 5))
```
