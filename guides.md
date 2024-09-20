
# Documentation

* for worskpace:
``` 
cargo doc --workspace --no-deps
```
* for individual members:
```
cargo doc --no-deps
```
### Attributes
* **open** documentation in the **browser** right after building docs:
    ```
  --open
    ```
* ignore documentation generation of dependencies (except worskpace members):
    ```
    --no-deps
    ```
  
# Unit testing
in case of failed unit test, terminal would show as follows
``` 
error: 1 target failed:
    `-p basics --test hierarchy`
``` 
### Specification 
* for worskpace:
  ``` 
  cargo test --no-fail-fast --workspace
  ```
* for individual members
  ``` 
  cargo test --no-fail-fast
  ```