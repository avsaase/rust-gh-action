



# rust-gh-action

GitHub Actions supports container-based actions but unfortunately it does not apply layer caching. With languages like Rust and it's famouly long compile times this leads to long workflow run times as it needs to recompile the binary on every run. This repo demonstrates how a composite action can be used to run any docker container while using layer caching.

## Inputs

### `url`

**Required** URL to make a HTTP GET request to. Default `"https://httpbin.org/get"`.

### `cache`
Whether or not to use layer caching for the Docker builds. Defautls to "true". If your Rust binary has few depenencies then recompiling it on every workflow run may be faster than fetching layers from the cache. Caching layers on the first run of the action can take a few minutes.

## Outputs

### `response`

JSON response from the GET request.

### `error`

Any error that occured with either making the request or decoding the response.

## Example usage
```yaml
uses: avsaase/rust-gh-action@master
with:
  url: '"https://httpbin.org/get"
```
