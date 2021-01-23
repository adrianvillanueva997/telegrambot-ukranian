![Build Docker image](https://github.com/adrianvillanueva997/telegrambot-ukranian/workflows/Build%20Docker%20image/badge.svg)
# telegrambot-ukranian
Improved version of the original bot.

# Deployment

The easiest way to deploy the project is by building the dockerfile and deploy it with docker-compose. Sample docker files can be found in the repository. To support /ukranian or /wikired commands, its needed to build wikired-api docker image. Otherwise those commands will not work. Also, it is needed to get an OpenWeather api key and create a new bot using botfather on telegram. Put those keys in a .env file.
