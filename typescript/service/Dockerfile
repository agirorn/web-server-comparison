FROM node:18

WORKDIR /usr/src/app

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
COPY package.json ./
COPY yarn.lock ./

RUN mkdir src
COPY src ./src
COPY tsconfig.json .

RUN yarn --frozen-lockfile
# # building your code for production
RUN yarn compile

EXPOSE 3000
CMD [ "node", "./dist/index.js" ]
