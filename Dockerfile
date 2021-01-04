# Multistage docker image building
# build-env -> dist

FROM node:14.9-alpine as base
# Building container
FROM base as builder
WORKDIR /build
COPY package.json .
RUN npm install
COPY . .
RUN npm run build

# Production container
FROM base as dist
WORKDIR /app
COPY package.json .
COPY --from=builder /build/dist ./dist
CMD ["npm", "run", "run"]