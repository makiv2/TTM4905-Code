# Project Name

## Description

This project aims to develop a proof-of-concept whistleblower electronic reporting
environment leveraging the Zero-Knowledge Virtual Machine (zkVM) SP1. By
utilizing zkVM technology, the students will implement Zero-Knowledge Proofs
(ZKPs) to ensure the anonymity and privacy of whistleblowers. This implementation
on the SP1 platform will demonstrate the feasibility of secure and confidential
whistleblowing mechanisms with zkVM. The initiative underscores the crucial role of
such technologies in promoting transparency and protecting personal freedoms.
More information can be found in the accompanying master's thesis.

## Running the System

### Option 1: Running the System with Docker

The entire system can be run using Docker with Docker Compose. To run individual subcomponents, use the `docker-compose.yml` files located in the `backend/` and `rocket/` subfolders.

#### Running the Entire System

To start the entire system, execute the following command in the root directory:

```bash
docker compose up
```

The webpage has to be run manually, navigate to the subfolder `frontend/` for more intructions.

#### Running subcomponents
To run specific subcomponents of the system, navigate to the respective subfolder `backend/` or `rocket/` and execute the same command:

```bash
docker compose up
```

### Option 2: Running the System Manually without Virtualization

Follow the instructions in each of the four subfolders `backend/`, `frontend/`,  `rocket/` or `sp1/` to run the system without the use of Docker. (This can be time consuming)

## Usage
The system can be demonstrated by the use of the webpage in `frontend/` or by manually calling on the endpoints in the Rocket Backend with Postman or similar software.

Website Pages
- `/admin/login` - Mock login page that takes you to the admin panel to view complaints and the accompanying proofs.
- `/generate` - To generate a complaint.

Rocket Endpoints
-
-
-

## Contributing

Contributions are welcome! Please follow these guidelines:

- Fork the repository.
- Create a new branch.
- Make your changes.
- Describe your changes and what they accomplish.
- Submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
