use solver::*;
use network::Network;

#[derive(Debug, Copy, Clone)]
/// Stochastic Gradient Descent Solver
pub struct SGD;

impl SGD {
    /// Return the current learning rate. The currently implemented learning rate
    /// policies are as follows:
    ///    - fixed: always return base_lr.
    ///    - step: return base_lr * gamma ^ (floor(iter / step))
    ///    - exp: return base_lr * gamma ^ iter
    ///    - inv: return base_lr * (1 + gamma * iter) ^ (- power)
    ///    - multistep: similar to step but it allows non uniform steps defined by
    ///      stepvalue
    ///    - poly: the effective learning rate follows a polynomial decay, to be
    ///      zero by the max_iter. return base_lr (1 - iter/max_iter) ^ (power)
    ///    - sigmoid: the effective learning rate follows a sigmod decay
    ///      return base_lr ( 1/(1 + exp(-gamma * (iter - stepsize))))
    ///
    /// where base_lr, max_iter, gamma, step, stepvalue and power are defined
    /// in the solver parameter protocol buffer, and iter is the current iteration.
    fn get_learning_rate(&self) -> f32 {
        // Dtype rate;
        // const string& lr_policy = this->param_.lr_policy();
        // if (lr_policy == "fixed") {
        //   rate = this->param_.base_lr();
        // } else if (lr_policy == "step") {
        //   this->current_step_ = this->iter_ / this->param_.stepsize();
        //   rate = this->param_.base_lr() *
        //       pow(this->param_.gamma(), this->current_step_);
        // } else if (lr_policy == "exp") {
        //   rate = this->param_.base_lr() * pow(this->param_.gamma(), this->iter_);
        // } else if (lr_policy == "inv") {
        //   rate = this->param_.base_lr() *
        //       pow(Dtype(1) + this->param_.gamma() * this->iter_,
        //           - this->param_.power());
        // } else if (lr_policy == "multistep") {
        //   if (this->current_step_ < this->param_.stepvalue_size() &&
        //         this->iter_ >= this->param_.stepvalue(this->current_step_)) {
        //     this->current_step_++;
        //     LOG(INFO) << "MultiStep Status: Iteration " <<
        //     this->iter_ << ", step = " << this->current_step_;
        //   }
        //   rate = this->param_.base_lr() *
        //       pow(this->param_.gamma(), this->current_step_);
        // } else if (lr_policy == "poly") {
        //   rate = this->param_.base_lr() * pow(Dtype(1.) -
        //       (Dtype(this->iter_) / Dtype(this->param_.max_iter())),
        //       this->param_.power());
        // } else if (lr_policy == "sigmoid") {
        //   rate = this->param_.base_lr() * (Dtype(1.) /
        //       (Dtype(1.) + exp(-this->param_.gamma() * (Dtype(this->iter_) -
        //         Dtype(this->param_.stepsize())))));
        // } else {
        //   LOG(FATAL) << "Unknown learning rate policy: " << lr_policy;
        // }
        // return rate;
        unimplemented!();
    }

    fn clip_gradients(&self) {
        // const Dtype clip_gradients = this->param_.clip_gradients();
        // if (clip_gradients < 0) { return; }
        // const vector<Blob<Dtype>*>& net_params = this->net_->learnable_params();
        // Dtype sumsq_diff = 0;
        // for (int i = 0; i < net_params.size(); ++i) {
        //   sumsq_diff += net_params[i]->sumsq_diff();
        // }
        // const Dtype l2norm_diff = std::sqrt(sumsq_diff);
        // if (l2norm_diff > clip_gradients) {
        //   Dtype scale_factor = clip_gradients / l2norm_diff;
        //   LOG(INFO) << "Gradient clipping: scaling down gradients (L2 norm "
        //       << l2norm_diff << " > " << clip_gradients << ") "
        //       << "by scale factor " << scale_factor;
        //   for (int i = 0; i < net_params.size(); ++i) {
        //     net_params[i]->scale_diff(scale_factor);
        //   }
        // }
    }
}

impl ISolver for SGD {
    fn apply_update(&self, param: &SolverConfig, net: &mut Network) {
        // CHECK(Caffe::root_solver()); // Caffe
        let rate = self.get_learning_rate();

        self.clip_gradients();
        for (param_id, param) in net.learnable_params().iter().enumerate() {
            //     Normalize(param_id);
            //     Regularize(param_id);
            //     ComputeUpdateValue(param_id, rate);
            unimplemented!();
        }
        net.update_params();

        unimplemented!();
    }
}