# Microservices based on gRPC with Rust

![](./assets/cover.png)

## Introduction

In the world of modern software development, building highly performant and scalable distributed systems is a common challenge. To tackle this, the protocol buffers and the gRPC framework has gained significant popularity due to its efficient communication protocol and language-agnostic nature.

### What is gRPC

gRPC is an open source framework for general-purpose RPCs across a network developed by Google.
gRPC supports full-duplex streaming and is also mostly aligned with HTTP/2 semantics. It supports different media formats, such as Protocol Buffers (Protobuf), JSON, XML, and Thrift. Protobuf is the default media format. The use of Protobuf aces the others because of higher performance.

### How does gRPC work

gRPC allows two applications in different processes or machines to communicate with each other as if they were in the same process. This is useful for distributed applications, such as microservices or high-availability systems.

### gRPC has the following advantages

  - Scalability: gRPC uses HTTP/2, which is an efficient protocol that allows scaling the performance of RPC calls.

  - Performance: gRPC uses Protocol Buffers, which is an efficient data description language that helps reduce the size of RPC messages.

  - Security: gRPC supports a variety of security mechanisms, such as authentication and encryption.

### When to use gRPC

gRPC is a good choice for the following applications:

  - Microservices: gRPC is a good choice for communication between microservices, as it allows scaling the performance and throughput of RPC calls.

  - High-availability systems: gRPC is a good choice for high-availability systems, as it allows applications to communicate with each other even if one of the applications fails.

  - Internal API integrations: gRPC is a good choice for internal API integrations, as it allows efficient communication between applications that are written in different programming languages.