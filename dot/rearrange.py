import cv2

image = cv2.imread('input.png')
blur = cv2.pyrMeanShiftFiltering(image, 11, 21)
gray = cv2.cvtColor(blur, cv2.COLOR_BGR2GRAY)
thresh = cv2.threshold(gray, 0, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)[1]

squares = []

cnts = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
cnts = cnts[0] if len(cnts) == 2 else cnts[1]
area_tresh_hold = 10000
for c in cnts:
    peri = cv2.arcLength(c, True)
    approx = cv2.approxPolyDP(c, 0.015 * peri, True)
    area = cv2.contourArea(c) 
    if len(approx) == 4 and area > area_tresh_hold:
        x,y,w,h = cv2.boundingRect(approx)
        cv2.rectangle(image,(x,y),(x+w,y+h),(36,255,12),2)
        square = image[y:y+h, x:x+w]
        squares.append(square)

cv2.imshow('thresh', thresh)
cv2.imshow('image', image)
cv2.waitKey()


