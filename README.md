
# cmasfo

CMASFO is a forum website that I'm planning to make.

And other people are always welcome to contribute.

## Main Repositories

* cmasfo
* cmasfo-dev
* cmasfo-ssg
* cmasfo-cms
* cmasfo-web
* cmasfo-app

## Development Cycle

### cmasfo

This is a main archive repo.

In here, lots of functionality will be tested paritally.

If some tests are successful, it can be implemented to cmasfo-dev, which is a rust crate.

### cmasfo-dev

This is main dev framework for other web or app repositories.

To enhance productivity, it's important to integrate development environment.

This also can be used as cmasfo cli application. (cargo install cmasfo-dev)

### cmasfo-ssg

First repo to test code is a ssg repository.

SSG just creates a static website.

And once it's completed, it bothers less to take care of.

### cmasfo-cms

Second repo to test code is a cms repository.

Unlike SSG, it needs to take care of the server for CMS.

But it will be good practice for making cmasfo-web or cmasfo-app.

### cmasfo-web

This should support two main features.

* Run main server
* Mirror the main server

I will run this as a main server.

And the server will be located in South Korea.

If the server is slow, a mirror site needs to be made easily.

### cmasfo-app

If cmasfo-dev is cli application, this is a gui application.

This should support two main features.

* Browse cmasfo-web with better features, and performance
* Support other features that couldn't be implemented to web version

And other partial features can be downloaded as modules.
