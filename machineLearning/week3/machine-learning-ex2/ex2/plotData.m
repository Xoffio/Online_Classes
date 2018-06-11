function plotData(X, y)
%PLOTDATA Plots the data points X and y into a new figure
%   PLOTDATA(x,y) plots the data points with + for the positive examples
%   and o for the negative examples. X is assumed to be a Mx2 matrix.

% Create New Figure
figure; hold on;

% ====================== YOUR CODE HERE ======================
% Instructions: Plot the positive and negative examples on a
%               2D plot, using the option 'k+' for the positive
%               examples and 'ko' for the negative examples.
%

posN = find(y==1); % find values that are equal to 0 and return index
negN = find(y==0); % find values that are equal to 1 and return index

plot(X(posN, 1), X(posN, 2), 'ko', 'MarkerFaceColor', 'g');
plot(X(negN, 1), X(negN, 2), 'ko', 'MarkerFaceColor', 'r');

% =========================================================================

hold off;

end
