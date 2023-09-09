## Mastodon Websocket Rust Example
This is a small program that demonstrates connecting to a Mastodon websocket.

### Getting Started

#### Does my instance support the streaming API?
Unfortunately, not every instance supports the streaming API so let's check yours. This is really easy to do with `curl`. You can run: `curl <YOUR_INSTANCE>/api/v2/instance`. As a concrete example, I'm using the botsin.space instance so I would run `curl https://botsin.space/api/v2/instance`.

The value you'll be looking for is `configuration.urls.streaming`. In my case, this is `wss://botsin.space`. We'll save this value for later.

#### What events are you subscribing to?
There are a set of constants to use depending on the events you want to receive. You can refer to [this link](https://docs.joinmastodon.org/methods/streaming/#streams) for what they are. In this case I'll be using `direct` to listen for direct messages to the bot account.

#### How do I get an access token?
- Go to the settings on your instance
- Click 'Development', then 'New Application'.
- Give this new application a name and (optionally) a link to a website.
- Refer to the [OAuth description for the event](https://docs.joinmastodon.org/methods/streaming/#direct) to find out what scope you need.
- You'll need the value from "Your access token" to pass as the environment variable.

#### Running this example
```bash
STREAMING_DOMAIN="wss://botsin.space" STREAM=direct ACCESS_TOKEN=<YOUR_ACCESS_TOKEN> cargo run
```

