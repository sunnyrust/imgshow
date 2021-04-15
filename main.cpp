#include<opencv2/opencv.hpp>
using namespace cv;

int main(){
        Mat img = imread("./love.jpg");
        imshow("LOVE", img);
        waitKey(0);
        return 0;
}
