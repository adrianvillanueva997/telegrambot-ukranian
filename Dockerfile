# Multistage docker image building
# build-env -> dist
# TODO do multistage building image
# For now it is single image building since it is in beta
FROM node:14.9-alpine
WORKDIR /app
COPY package.json .
RUN npm install
COPY . .
CMD ["npm","start"]

