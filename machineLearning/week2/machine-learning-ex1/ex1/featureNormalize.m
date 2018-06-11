function [X_norm, mu, sigma] = featureNormalize(X)
%FEATURENORMALIZE Normalizes the features in X
%   FEATURENORMALIZE(X) returns a normalized version of X where
%   the mean value of each feature is 0 and the standard deviation
%   is 1. This is often a good preprocessing step to do when
%   working with learning algorithms.

% You need to set these values correctly
X_norm = X;
mu = zeros(1, size(X, 2));
sigma = zeros(1, size(X, 2));


% ====================== YOUR CODE HERE ======================
% Instructions: First, for each feature dimension, compute the mean
%               of the feature and subtract it from the dataset,
%               storing the mean value in mu. Next, compute the
%               standard deviation of each feature and divide
%               each feature by it's standard deviation, storing
%               the standard deviation in sigma.
%
%               Note that X is a matrix where each column is a
%               feature and each row is an example. You need
%               to perform the normalization separately for
%               each feature.
%
% Hint: You might find the 'mean' and 'std' functions useful.
%


mu = mean(X);
%disp(mu(1,1));
%disp(X_norm(:,2)-mu(1,2));
%X_norm = [X_norm(:,1)-mu(1, 1), X_norm(:,2)-mu(1, 2)];
disp('mean:');
disp(mu);
%disp(X_norm);
sigma = std(X);
disp('sigma');
disp(sigma);
disp('---');

%sigma = [X(:,1)/sigma(1,1), X(:,2)/sigma(1,2)];
%disp(sigma);
maxV1 = max(X_norm(:,1));
maxV2 = max(X_norm(:,2));
disp(maxV1);
disp(maxV2);
%X_norm = sigma;

X_norm = [X_norm(:,1).-mu(1,1), X_norm(:,2).-mu(1,2)];
X_norm = [X_norm(:,1)./sigma(1,1), X_norm(:,2)./sigma(1,2)];

%X_norm = [ (X(:,1))/maxV1, (X(:,2))/maxV2];






% ============================================================

end
