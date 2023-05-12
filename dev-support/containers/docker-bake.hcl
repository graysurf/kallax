variable "TAG" {
    default = "develop"
}

variable "REPOSITORY" {
    default = "886360478228.dkr.ecr.us-west-2.amazonaws.com"
}

variable "DEBUG" {
    default = "0"
}

group "default" {
    targets = [
        "kallax",
    ]
}

target "base" {
    dockerfile = "dev-support/containers/debian/Containerfile"
    args = {
      DEBUG = "${DEBUG}"
    }
    platforms = ["linux/amd64"]
}

target "kallax" {
    inherits = ["base"]
    target = "kallax"
    tags = ["${REPOSITORY}/thxnet/kallax:${TAG}"]
}