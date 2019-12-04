# RustyBot
A slack bot that greets people

# Usage

This bot uses the Event API which allows it to cope with much higher traffic if
needed. This means that the bot runs a webserver which needs to be accessible
from the internet by slack.

So on docker, the port 8000 inside the container needs to be forwarded to the
host. In the Event API settings on the slack interface, the event
`member_joined_channel` needs to be subscribed to, and the request URL needs to
be set to that of the bot.

The environment variable `SLACK_TOKEN` needs to be set to the Oauth token.

## Example

```sh
$ git clone https://github.com/NilsIrl/RustyBot.git && cd RustyBot
$ docker build -t rusty-bot .
$ docker container run -d -p 80:8000 -e SLACK_TOKEN=<token> rusty-bot
```

In the above example the port exposed to the internet, on the host, is the port
`80`.
