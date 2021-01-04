# Multistage docker image building
# build-env -> dist

FROM node:14.9-alpine as base
# Build container
FROM base as build-env
RUN apk --no-cache update && apk add python make g++ && rm -rf
WORKDIR /build
COPY package.json .
COPY .babelrc .
RUN npm install
COPY . .
RUN npm run build

# Production container (Dist)
FROM base as dist
WORKDIR /app
COPY package.json ./
COPY .babelrc ./
COPY .env .
RUN npm install
COPY --from=build-env /build/dist ./dist
CMD [ "npm", "start" ]