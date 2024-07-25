import { exec } from "child_process";
import fs from "fs-extra";
import dotenv from "dotenv";

const BACKEND_HOST = "http://localhost:8000";

// Output folder for the generated client
const OUTPUT_FOLDER = "./src/api/my_api";

// Path to the .env file
const ENV_FILEPATH = "./../.env";

// Load environment variables from .env file
dotenv.config({ path: ENV_FILEPATH });

// Remove the folder if it exists
fs.remove(OUTPUT_FOLDER, (err) => {
    if (err) return console.error(`Error removing folder: ${err}`);
    console.log(`Removed folder: ${OUTPUT_FOLDER}`);

    if (!BACKEND_HOST) {
        console.error("Error: BACKEND_HOST is not defined in the .env file.");
        return;
    }

    // Run the client generation command
    exec(
        `openapi-generator-cli generate -i ${BACKEND_HOST}/api/openapi.json -g typescript-axios -o src/api/my_api --package-name my-api`,
        (error, stdout, stderr) => {
            if (error) {
                console.error(`Error executing command: ${error}`);
                return;
            }
            if (stdout) {
                console.log(`stdout: ${stdout}`);
            }
            if (stderr) {
                console.error(`stderr: ${stderr}`);
            }
        }
    );
});