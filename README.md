# acquirust

Acquirust is a Rust library for interacting with acquiring services. It currently consists of 4 crates:

- `acquiconnect` -  main trait for generalizing API actions behavior.
- `acquisim` - a simple bank simulator, designed to run in a Docker container.
- `acquisim-api` - bindings for acquisim.
- `mapi` - bindings for the [Merchant API](https://www.tinkoff.ru/kassa/dev/payments/#section/Vvedenie).

Additionally, there is [acqui](https://github.com/ghashy/acqui), written in Swift for macOS, which serves as an acquisim management client.

The primary purpose of acquisim is for mocking and running backends that need to process payments in a test environment.

> It is not finished, and currently only the `Init` method is supported. In the future, there are plans to implement other methods and potentially encapsulate all methods into a single interface with various pluggable backends.

This library is being developed concurrently with [harmonysphere](https://harmonysphere.ru), and all features will be gradually made available.

### Currently supported:

- [x] Standard non-PCI-DSS, non-recurrent, one-stage payment without 3DS checking.

### Planned:

- [ ] Recurrent payments
- [ ] Two-stage payments
- [ ] Notifications

### Methods:

- [ ] Get payment status method
- [ ] Get order status method
- [ ] Get operation info method

### PCI-DSS authorized methods:

- [ ] Check 3DS method
- [ ] Finish authorize method
