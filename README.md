# Mailmeld

Mailmeld is a simple command-line tool for performing mail merges. It allows you to send personalized emails to a list of recipients by specifying the subject, message, and recipients. You can either preview the emails before sending or directly send them.

## Installation

Clone the repository and build the project (if applicable):

```bash
git clone https://github.com/yourusername/mailmeld.git
cd mailmeld
cargo install --path .
```

## Usage

Mailmeld is used from the command line. The general syntax is:

```bash
mailmeld [OPTIONS] <SUBJECT> <MESSAGE> <COMMAND>
```

## Commands

- `send` : Send the emails to the specified recipients.
- `preview` : Preview the emails before sending them.

## Arguments

- `<SUBJECT>` : The subject line for the email.
- `<MESSAGE>` : The message body of the email. You can include placeholders like {name} which will be replaced with the recipient's information.

## Options

- `--recipients <RECIPIENTS>` : A csv file containing a list of recipients, and any additional variables.
- `-h, --help` : Print help information and exit.
- `-V, --version` : Print the version of Mailmeld and exit.

## Environment Variables

- `SMTP_USERNAME`: The username for the SMTP server.
- `SMTP_PASSWORD`: The password for the SMTP server.
- `SMTP_RELAY`: The SMTP relay server address.
- `SMTP_FROM`: The email address to send the emails from.
- `SMTP_REPLY_TO`: The email address to reply to.

## Personalisation

Additional information from the recipients option can be used to personalise emails.
Place a column header in curly braces with an optional default, i.e. `Hello {name|there}!`.
If the column header is not found in the recipients file, the default will be used.
You can use as many braces as you want, i.e. `Hello {{{name|there}}},\nWe have an important update for you. Your email is {{email}}.` is valid.

## Examples

### Send Emails

To send emails with a subject and message:

```bash
mailmeld "Important Update" "Hello {name},\nWe have an important update for you." send --recipients recipients.txt
```

### Preview Emails

To preview the emails before sending them:

```bash
mailmeld "Important Update" "Hello {name},\nWe have an important update for you." preview --recipients recipients.txt
```

## Contributing

If you'd like to contribute to Mailmeld, feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
