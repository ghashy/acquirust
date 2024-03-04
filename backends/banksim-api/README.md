# banksim-api

Bindings for [banksim](https://github.com/ghashy/banksim)

Additionally, there is [acqui](https://github.com/ghashy/acqui), written in Swift for macOS, which serves as an banksim management client.

> It is not finished. In the future, there are plans to implement other methods and potentially encapsulate all methods into a single interface with various pluggable backends.

This library is being developed concurrently with [harmonysphere](https://harmonysphere.ru), and features will be gradually made available.

### Currently supported:

- [x] Standard non-PCI-DSS, non-recurrent, one-stage payment without 3DS checking.
- [x] Card token registration for card-related operations without PCI-DSS.
- [x] Split payment
- [x] Direct payment

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
