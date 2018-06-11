function [C, sigma] = dataset3Params(X, y, Xval, yval)
%DATASET3PARAMS returns your choice of C and sigma for Part 3 of the exercise
%where you select the optimal (C, sigma) learning parameters to use for SVM
%with RBF kernel
%   [C, sigma] = DATASET3PARAMS(X, y, Xval, yval) returns your choice of C and
%   sigma. You should complete this function to return the optimal C and
%   sigma based on a cross-validation set.
%

% You need to return the following variables correctly.
C = 1;
sigma = 0.3;

% ====================== YOUR CODE HERE ======================
% Instructions: Fill in this function to return the optimal C and sigma
%               learning parameters found using the cross validation set.
%               You can use svmPredict to predict the labels on the cross
%               validation set. For example,
%                   predictions = svmPredict(model, Xval);
%               will return the predictions on the cross validation set.
%
%  Note: You can compute the prediction error using
%        mean(double(predictions ~= yval))
%

tmpC = [0.01, 0.03, 0.1, 0.3, 1, 3, 10, 30];
tmpSigma = [0.01, 0.03, 0.1, 0.3, 1, 3, 10, 30];
%predictions = zeros(length(tmpC), length(tmpSigma));
%pError = zeros(length(tmpC), length(tmpSigma));
oldpError = 10000;
%x1 = X(:,1);
%x2 = X(:,2);

for countC=1:length(tmpC)
  for countSigma=1: length(tmpSigma)

    model = svmTrain(X, y, tmpC(countC), @(x1, x2) gaussianKernel(x1, x2, tmpSigma(countSigma)));

    predictions = svmPredict(model, Xval);
    pError = min(mean(double(predictions ~= yval)));
    if (pError < oldpError)
      oldpError = pError;
      C = tmpC(countC);
      sigma = tmpSigma(countSigma);
    end

    fprintf(' \n %u-%u', countC, countSigma);
  end
end

fprintf('C: %f \n sigma: %f \n\n-----\n', C, sigma);

% =========================================================================

end
