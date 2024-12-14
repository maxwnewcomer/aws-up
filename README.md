# aws-up

An easy way to test code on different instances without losing track of "costly" resources.

## Quickstart

AWS up can be used by:

1. Getting access keys to appropriate account
2. Use `aws-vault` to add keys to a profile
   1. `aws-vault add personal`
3. To spin up run

   ```shell
   aws-vault exec <profile> --region <region> -- tofu apply
   ```

4. To spin down run

   ```shell
   aws-vault exec <profile> --region <region> -- tofu destroy
   ```

---

Hoping to build a little rust cli to make choosing an image, region + other settings way easier. Sick of the AWS management console. Actually baffled that they thought round corners would satisfy people.
