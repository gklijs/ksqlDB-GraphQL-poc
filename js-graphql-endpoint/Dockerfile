FROM node:14.16.0-alpine3.13

WORKDIR /root/app
COPY . .

RUN yarn install

ENTRYPOINT yarn run start