mod services;
mod libs;

use services::airsens;


fn main() {
    airsens::run();
}
